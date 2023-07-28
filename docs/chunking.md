# Chunking

*The goal of this document is to explain how chunking works.*

## High-Level Overview and Goals

The purpose of the chunking system is to ease the management of world state by splitting the world into fixed-size *chunks*, encoded as a `world::Chunk` struct. These chunks then become the basic primitive for representing, storing, loading and communicating the world's state. The game driver maintains a set of currently loaded *chunks*, it treats *chunks* (well, data associated with them) as the primitive for rendering, and loads *chunks* from a file (or generates new ones if the chunk in question has never been visited before) as the player moves around the game world.

A `Chunk` intentionally unifies two ideas: a chunk as understood semantically by the game world, containing a fixed-size array of blocks and lighting levels; and a renderable world object consisting of vertex data and texture indices. Chunks are explicitly *not* part of the game engine: they are defined in the `world` crate, on which both the client and server binaries depend. The game engine sees them no differently to any other type of model.

## Client-Side Chunk Management

The game client maintains a *window* of currently-loaded chunks. It stores loaded chunks in a 2D circular buffer. As the player moves around the world, this window updates, replacing far-away chunks with closer ones whenever the player crosses a boundary. This ensures that, at all points in time, a minimum radius of renderable chunks exists around the player. Additionally, using a fixed-size "radius" of loaded blocks avoids the need to make dynamic memory allocations when loading new chunks, since new blocks will always replace the previous chunk in the buffer that's now too far away to be worth rendering.

TODO: Talk about caching a larger window than the renderable set

The `ChunkManager` struct implements the process of loading chunks from a hierarchy of repositories., or generating them if the chunk is new. 

## The Engine's View

The separate blocks in a chunk are merged into a single 
