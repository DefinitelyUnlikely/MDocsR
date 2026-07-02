# MDocsR
A Rust version of my MDocs C# backend that implemented [YDotNet](https://github.com/y-crdt/ydotnet) to match with a Yjs frontend - leveraging the fact that [Yrs/y-crdt](https://github.com/y-crdt/y-crdt) has a few more features and that rust is usually a bit faster than C#, which can make a difference in a read/write heavy environment like a multiplayer text-editor. 

### Built with
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white&style=for-the-badge)](#)  <br>
[![Postgres](https://img.shields.io/badge/Postgres-%23316192.svg?logo=postgresql&logoColor=white&style=for-the-badge)](#)  <br>
[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff&style=for-the-badge)](#)  <br>
[![Y.js](https://img.shields.io/badge/Yjs-fff?logo=yjs&style=for-the-badge)](https://yjs.dev/)  <br>

## What is MDocs and MDocsR?

MDocs is a real-time co-op text-editor I prototyped as part of my thesis. MDocsR is a project I started as a way to learn more about Rust but also bring MDocs out of the prototype stage and into a more production ready state. The end goal is that MDocs will be part of a larger ecosystem of projects, most in some way or other relating to common office programs, such as Google Docs/Word, Sheets/Excel, Slides/PowerPoint. 

The project aims to make use of primarily open-source technologies and, when in a more production ready environment, not have to really on tech owned by American companies (Not hosting this with, for example, Azure or making use of Google SSO) as the thesis revolved around the cost and requirements around creating european alternatives to common everyday software like Google Docs.  

## In the works

As of currently, I am translating work already done in .NET over to rust. The work is done "slowly" as a way to learn about Rust and not being done as a "simple" port. A few improvements are being implemented while the backend is being refactored anyway. See below.

### Passkeys

The prototype used passwords as a means to login during testing. I started out with passwords for the rust version as well. As of currently, I am refactoring the application to instead use passkeys - as passwords are less secure without MFA. 
