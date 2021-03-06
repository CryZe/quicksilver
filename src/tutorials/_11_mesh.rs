//! A `Mesh` is a low-level graphics concept in Quicksilver consisting of a series of polygon
//! vertices and a list of triangles.
//!
//! This concept can be used to draw any shape: to make a
//! rectangle, put the four vertices in the vertex list, and then create two triangles that
//! together make up the rectangle.
//!
//! Most user code need never encounter a mesh, but `Mesh` is extremely powerful. In fact, there is
//! no special sauce available to Quicksilver's implementation of `Drawable` for `Rectangle` that a
//! regular user of the library cannot access with a Mesh.
//!
//! There are two ways to use `Mesh`: instantiate your own with `Mesh::new` or use `Window::mesh` to
//! access the internal mesh of a window. To concatenate two meshes, use the `Mesh::extend` function.
//!
//! To create a mesh that contains a triangle with a red vertex, a blue vertex, and a green vertex,
//! you could write:
//!
//! ```no_run
//! use quicksilver::graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex};
//! let vertices = vec![
//!     Vertex::new((400, 200), None, Col(Color::RED)),
//!     Vertex::new((200, 400), None, Col(Color::BLUE)),
//!     Vertex::new((600, 400), None, Col(Color::GREEN))
//! ];
//! let triangles = vec![ GpuTriangle::new(0, [0, 1, 2], 0.0, Col(Color::WHITE)) ];
//! let mesh = Mesh { vertices, triangles };
//! ```
//!
//! To draw it you can then do:
//!
//! ```no_run
//! # use quicksilver::{graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex}, lifecycle::Window };
//! # fn func(window: &mut Window) {
//! # let vertices = vec![
//! #     Vertex::new((400, 200), None, Col(Color::RED)),
//! #     Vertex::new((200, 400), None, Col(Color::BLUE)),
//! #     Vertex::new((600, 400), None, Col(Color::GREEN))
//! # ];
//! # let triangles = vec![ GpuTriangle::new(0, [0, 1, 2], 0.0, Col(Color::WHITE)) ];
//! # let mesh = Mesh { vertices, triangles };
//! window.mesh().extend(&mesh);
//! # }
//! ```
//!
//! You have to do this every frame, because the window's mesh is cleared after it's drawn.
