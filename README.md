## Bevy Reactive Font

A way of storing fonts and default text information and then refering back to them when writing text.

With any luck this should make writing long sections of text really quick.

It also pairs well with my [bevy_html_lite](https://github.com/Freyja-moth/bevy_html_lite) to make things even easier.

Check out [examples/basic](https://github.com/Freyja-moth/bevy_reactive_font/blob/main/examples/basic.rs) to get an idea of how it works.

## There is currently a breaking bug
Due to the way that bevy handles despawning, the code that handles the removal of `ReactiveFont` will be suplied an entity that has already been despawned
