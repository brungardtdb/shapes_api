# Shapes API # 🦀

The shapes API is REST API (under development) written in the Rust programming language that enables users to easily get information about steel shapes. Initially only AISC steel shapes will be supported. It's possible that other shapes such as aluminum shapes will be supported in the future. 

The goal is to eventually provide the following:

- A general library for modeling and mapping shapes
- A library that will act as a repository for shapes, managing CRUD operations with the database
- A REST API that will allow a user to request data for a given shape or shapes using HTTP
- A CLI tool that will allow a user to request data for a given shape or shapes from the command line, behind the scenes this will make calls to the API
- A web UI allowing the user to display cross sections for shapes in the browser (as a stretch goal, it will allow the users to view a 3D rendering of each shape)

