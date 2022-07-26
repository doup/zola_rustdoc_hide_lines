# This is some example

Lorem ipsum dolor sit amet consectetur, adipisicing elit.
Autem commodi necessitatibus vel, harum fuga saepe libero doloribus ad in nisi minus rerum iste at quos dolorem ipsa quia.
Pariatur, minus.

```rust
# use bevy::ecs::prelude::*;
# #[derive(Component)]
# struct Life;
#
# #derive(Component)
# struct Defense;
// A system that has write-access to the Life component, and read-access to the Defense component
// Only entities with both Life and Defense will be included in this Query
fn life_and_defense(mut query: Query<(&mut Life, &Defense)>) {}
```

Ab mollitia perferendis harum ad fugiat maiores quos dolorum debitis! Unde dolorem fugit exercitationem quibusdam ad id rerum veritatis dolor impedit magnam, maiores assumenda iste consequatur a aspernatur recusandae dolores consectetur quam.

This one has mismatched annotation:

```rust,hide_lines=1-2
# use bevy::ecs::prelude::*;
# #[derive(Component)]
# struct Life;
#
# #derive(Component)
# struct Defense;
// A system that has write-access to the Life component, and read-access to the Defense component
// Only entities with both Life and Defense will be included in this Query
fn life_and_defense(mut query: Query<(&mut Life, &Defense)>) {}
```

Some code without hidden lines:

```rust
struct NoHiddenLines;
```
