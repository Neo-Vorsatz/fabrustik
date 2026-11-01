# FabRUSTik: Inverse kinematics solver for serial manipulators

## Overview
This library is for solving inverse kinematics of serial manipulators, using a variation of FABRIK described by [Moreno *et al.*](https://doi.org/10.1109/ICEV56253.2022.9959663). It allows the user to model several serial manipulators using different joint types, and determine actuation values for the end-effector to reach a desired target.

This library originated in a project for the University of Cape Town:<br>
[Secure Teleoperation of a Robotic Arm using Computer Vision](https://github.com/Neo-Vorsatz-UCT/eee4022s)

## Credits
[Moreno *et al.*](https://doi.org/10.1109/ICEV56253.2022.9959663) receive credit for developing the base algorithm.

## Features
### Target
- desired 3D position (required)
- desired 3D direction (required)
- desired rotation (required)

### Joint Types
- hinge joints
- pivot joints
- prismatic joints

### Joint Features
- range of motion constraints

### Computation
- fixed point arithmetic (48 integer bits, 16 fractional bits)
- uses previous solution as initial guess

## Visualization
<image src="media/joint_diagrams.png" controls width="100%"></image>

## Future Features
If I have the time in future, or if someone is willing to contribute to this library, here are some possible improvements:
- Support parallel robotic manipulators
- Support more joint types (ball, sliding, screw)
- Support orientationless (only position) targets
- Add optimisation for fixed joints; joints that don't move and only provide a fixed positional or rotational offset
- Add positional boundaries, in which joints must remain or never enter (useful for modelling the ground)

## License
This library is licensed under the GNU Lesser General Public License v2.1.
