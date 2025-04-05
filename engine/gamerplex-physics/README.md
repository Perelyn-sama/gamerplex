## Architecture ideas

- Rigid bodies(static, dynamic and kinematic)
- collisions shapes(primitives like sphere, box, pryramids to complex meshes)
- contrainsts and joints(hinges, ball joints and fixed joints)
- forces and impulses
- collision and resolution

### Integration with ECS
- Physics components (RigidBody, Collider)
- Physics systems (MovementSystem, CollisionSystem)
- Event systems for collision callbacks

### Performance Considerations
- Spatial partitioning (octree, BVH)
- Broad phase (detect potential collisions)
- Narrow phase (precise collision detection)
- Parallel computation where possible