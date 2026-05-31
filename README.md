# MDocsR
A Rust version of my MDocs C# backend that implemented [YDotNet](https://github.com/y-crdt/ydotnet) to match with a Yjs frontend - leveraging the fact that [Yrs/y-crdt](https://github.com/y-crdt/y-crdt) has a few more features and that rust is usually a bit faster than C#, which can make a difference in a read/write heavy environment like a multiplayer text-editor. 

## What is MDocs and MDocsR?

MDocs is a real-time co-op text-editor I prototyped as part of my thesis. MDocsR is a project I started as a way to learn more about Rust but also bring MDocs out of the prototype stage and into a more production ready state. The end goal is that MDocs will be part of a larger ecosystem of projects, most in some way or other relating to common office programs, such as Google Docs/Word, Sheets/Excel, Slides/PowerPoint. 
