# Controls

TODO: Talk about the plan to implement:

```rust
trait EventSource {
    fn poll_events() -> Iterator<Item=engine::Event>;
}

enum GameEvent {
    MovePlayer(...),
    PlaceBlock(...),
    RemoveBlock(...),
    CloseGame
}

impl<T> Controls<T> where T: EventSource {
    ...
    fn consume_events(&mut self, source: &mut T) -> Iterator<Item=GameEvent> { ... }
}
```

In short, the purpose of `Controls` will be to map from "engine events" (e.g. keyboard key press) to higher-level semantic "game events". It maps a stream of engine events to a stream of game events, with some state involved to remember things like which keys are currently held.