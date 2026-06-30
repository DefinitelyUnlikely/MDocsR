# MDocsR
A Rust version of my MDocs C# backend that implemented [YDotNet](https://github.com/y-crdt/ydotnet) to match with a Yjs frontend - leveraging the fact that [Yrs/y-crdt](https://github.com/y-crdt/y-crdt) has a few more features and that rust is usually a bit faster than C#, which can make a difference in a read/write heavy environment like a multiplayer text-editor. 

### Built with
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white&style=for-the-badge)](#)  <br>
[![Postgres](https://img.shields.io/badge/Postgres-%23316192.svg?logo=postgresql&logoColor=white&style=for-the-badge)](#)  <br>
[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff&style=for-the-badge)](#)  <br>
[![Y.js](https://img.shields.io/badge/Yjs-fff?logo=yjs&style=for-the-badge)](https://yjs.dev/)  <br>

## What is MDocs and MDocsR?

MDocs is a real-time co-op text-editor I prototyped as part of my thesis. MDocsR is a project I started as a way to learn more about Rust but also bring MDocs out of the prototype stage and into a more production ready state. The end goal is that MDocs will be part of a larger ecosystem of projects, most in some way or other relating to common office programs, such as Google Docs/Word, Sheets/Excel, Slides/PowerPoint. 


