//! Example using serial manipulator with two joints
//! Author: Neo Vorsatz
//! Last updated: 1 November 2026

use fabrustik::{SerialRobot, JointType};

fn main() {
    // Create a robot with 2 joints
    let mut robot: SerialRobot = SerialRobot::new(2);

    // Set the base (location of first joint)
    robot.set_base(
        [0.0, 0.0, 3.0], // position
        [0.0, 0.0, 1.0], // direction
        [1.0, 0.0, 0.0] // rotation
    );

    // Describe first joint
    robot.conf_joint(
        JointType::Pivot, // pivot joint
        68.0, // 68 units until next joint
        false, // unlimited range of motion (full 360 degrees)
        0.0, // minimum angle (not used)
        0.0 // maximum angle (not used)
    );
    robot.next_joint(); // increment internal index

    // Describe second joint
    robot.conf_joint(
        JointType::Hinge, // hinge joint
        123.0, // 123 units until end-effector
        true, // limited range of motion
        -75.0, // minimum angle is -75 degrees
        75.0 // maximum angle is 75 degrees
    );

    // Set target (not, this solution isn't perfectly achievable)
    let target_pos:[f32; 3] = [100.0, 100.0, 100.0];
    let target_dir:[f32; 3] = [1.0, 1.0, 1.0]; // normalized internally
    let target_rot:[f32; 3] = [1.0, -1.0, 0.0]; // normalized internally
    robot.set_target(target_pos, target_dir, target_rot);

    // Get solution
    let accept_error: f32 = 0.1; // set acceptable error to 0.1 squared units
    let max_iterations: usize = 10; // set maximum number of iterations to 10
    let actuations: Box<[f32]> = robot.solve(accept_error, max_iterations);
    println!("Pivot angle: {:.1} degrees", actuations[0]);
    println!("Hinge angle: {:.1} degrees", actuations[1]);

    // Evaluate solution
    let iterations: usize = robot.get_num_iterations(); // get number of iterations used in previous solution
    let error: f32 = robot.get_error(); // get remaining squared error
    println!("Used {} iterations to reach {:.3} squared error.", iterations, error);
}
