use r2r::{
    eagleye_msgs::msg::Position,
    geometry_msgs::msg::{Point, Pose, Quaternion},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub car_id: u32,
    pub busy: bool,
    pub assigned_task: Option<u32>, // 0 indicates no task is assigned
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoTable {
    // pub vehicle_state: Vec<VehicleState>, // current exist car
    pub task: HashMap<u32, Task>, // current task
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: u32,
    pub cur_location: Pose, // todo fill in location
    pub des_location: Pose, // todo fill in location
    pub timestamp: Option<i64>,
    pub assigned_car: Option<u32>, // 0 indicates no car is assigned
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallTaxi {
    pub task_id: u32,
    pub cur_location: Pose, // todo fill in location
    pub des_location: Pose, // todo fill in location
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequireTask {
    pub task_id: u32,
    pub car_id: u32,
    pub timestamp: i64,
}

impl VehicleState {
    pub fn new(car_id: u32) -> Self {
        VehicleState {
            car_id,
            busy: false,
            assigned_task: None,
        }
    }
}

impl InfoTable {
    pub fn new() -> Self {
        Self {
            task: HashMap::new(),
        }
    }

    // used for RSU to recevie taxi call
    pub fn called_taxi(&mut self, request: CallTaxi) {
        self.task.entry(request.task_id).or_insert_with(|| Task {
            task_id: request.task_id,
            cur_location: request.cur_location,
            des_location: request.des_location,
            timestamp: None,
            assigned_car: None,
        });
    }

    pub fn merge_task(&mut self, task: RequireTask) {
        let Some(my_task) = self.task.get_mut(&task.task_id) else {
            //indicate RequireTask arrive before CallTaxi. Car will not compete for this task, becasue its timestamp must be later.
            self.task.insert(
                task.task_id,
                Task {
                    task_id: task.task_id,
                    cur_location: Pose {
                        position: Point {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: Quaternion {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            w: 0.0,
                        },
                    },
                    des_location: Pose {
                        position: Point {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: Quaternion {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            w: 0.0,
                        },
                    },
                    timestamp: None,
                    assigned_car: Some(task.car_id),
                },
            );
            return;
        };

        if my_task.assigned_car.is_none() {
            my_task.assigned_car = Some(task.car_id);
            my_task.timestamp = Some(task.timestamp);
        } else if let Some(timestamp) = my_task.timestamp {
            if timestamp > task.timestamp {
                my_task.assigned_car = Some(task.car_id);
                my_task.timestamp = Some(task.timestamp);
            }
        }
    }
}

struct LatLon {
    cur: f64,
    des: f64,
}
