
pub struct LocalExecutor {
    task: Task,
    process: Popen,
}

type ActionResult = Option<State>;

impl LocalExecutor {

    pub fn is_dead(&self) -> bool{
        self.task.state.status == Status::Exited
    }

    pub fn start(task: Task) -> Result<Self, String> {
        let mut task = task;
        let payload = &task.payload;

        let mut cmd = Exec::cmd(&payload.cmd).args(&payload.args);
        if let Some(cwd) = &payload.cwd {
            cmd = cmd.cwd(cwd);
        }
        if let Some(env) = &payload.env {
            cmd = cmd.env_extend(&env)
        }

        let process = cmd.popen();

        match process {
            Ok(process) => {
                task.state.status = Status::Running;
                task.state.started_at = Some(Utc::now());
                task.state.pid = process.pid();
                Ok(LocalExecutor { process, task })
            }
            Err(e) => {
                Err(format!("failed to start {:?}", e))
                // let mut state = task.state;
                // state.error = Some(format!("failed to start {:?}", e));
                // state.finished_at = Some(Utc::now());
                // state.status = Status::Exited;
                // state.pid = None;
                // Some(state)
            }
        }
    }

    pub fn terminate(&mut self) {
        match self.process.terminate() {
            Ok(_) => {
                // we are dead
            }
            Err(e) => {
                self.task.state.add_error(format!("Terminate failed {}", e));
            }
        };
    }

    pub fn kill(&mut self) {
        match self.process.kill() {
            Ok(_) => {
                // we are dead
            }
            Err(e) => {
                self.task.state.add_error(format!("Kill failed {}", e));
            }
        };
        self.poll();
    }

    // pub fn stop(&mut self) {
    //     if let Error(e) = self.process.terminate(){
    //         self.task.state = Status::Exiting;
    //     }
    //     let mut state = task.state.clone();
    //     if let Some(x) = &self.process {
    //         state.status = Status::Exiting;
    //         Some(state)
    //     } else {
    //         state.status = Status::Dead;
    //         state.error = Some("Tried to stop a process that never started.".to_string());
    //         Some(state)
    //     }
    // }

    pub fn poll(&mut self) {
        let mut state = &mut self.task.state;
        state.pulse = Some(Utc::now());

        if state.status == Status::Running {
            if let Some(exit_status) = self.process.poll() {
                state.status = Status::Exited;
                state.finished_at = Some(Utc::now());
                match exit_status {
                    ExitStatus::Exited(code) => {
                        state.exit_code = Some(code.into());
                    }
                    ExitStatus::Signaled(code) => {
                        state.exit_code = Some(code.into());
                    }
                    ExitStatus::Other(code) => {
                        state.exit_code = Some(code.into());
                        state.error = Some(format!("Ended with Other({})", code));
                    }
                    ExitStatus::Undetermined => {
                        state.error = Some("Ended with some Undetermined error code".to_string());
                    }
                }
            }
        }
    }

    pub fn get_state(&self) -> &State {
        &self.task.state
        // let mut state = task.state.clone();
        // if let Some(x) = &self.process {
        //     let exit_status = x.poll();
        //     state.status = Status::Exiting;
        // } else {
        //     state.status = Status::Dead;
        // }
        // Some(state)
    }

    pub fn remove(mut self) {
        match &self.task.state.status {
            Status::Created => {
                // don't need to do anything.
                // though this state is impossible
            }
            Status::Exited => {
                // im already good to clean up
            }
            Status::Running => self.kill(),
        }
    }
}

struct TaskManager {
    executors: Vec<LocalExecutor>,
}

impl TaskManager {
    pub fn tick(&mut self){
        self.executors.iter_mut().for_each(|e| e.poll());
        //self.executors.drain_filter(|e| !e.is_dead()).for_each(|x|x.remove());
        self.executors.retain(|e| !e.is_dead() );
        self.executors.iter().map(|e|e.get_state());
    }

    pub fn shutdown(&mut self){
        //self.executors.drain(range);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_task() {
        assert_eq!(3, 4);
    }
}
