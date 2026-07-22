# gRPC-CrossLang-Communication-Demo
Minimal cross-language gRPC demo illustrating service-to-service communication between C++, Rust, and Go using Protobuf & FlatBuffers payloads.

<div align="center">
	<img src="img/grpc.svg" width="300px"/>
</div>

## Tech Stack

![Go](https://img.shields.io/badge/Go-00ADD8?style=for-the-badge&logo=go&logoColor=white)
![C++](https://img.shields.io/badge/C++-00599C?style=for-the-badge&logo=cplusplus&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Fiber](https://img.shields.io/badge/Fiber-00ACD7?style=for-the-badge)
![gRPC](https://img.shields.io/badge/gRPC-4285F4?style=for-the-badge&logo=grpc&logoColor=white)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-005F9E?style=for-the-badge)
![Jet](https://img.shields.io/badge/Jet-FF5A5F?style=for-the-badge)
![Axios](https://img.shields.io/badge/Axios-5A29E4?style=for-the-badge&logo=axios&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)
![Alpine.js](https://img.shields.io/badge/Alpine.js-77C1D2?style=for-the-badge&logo=alpine.js&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)

## Architecture & Communication Flow

![Microservices Architecture Flow](img/microservices-architecture-flow.png)

### Request & Response Sequence

> 1. **Initiation**: The client browser sends an **AJAX request** to the Golang HTTP Server (API Gateway).
> 2. **Gateway to C++ (Protobuf)**: The HTTP Server initiates a gRPC call exclusively to the **C++ Service** using **Protobuf**.
> 3. **C++ to Rust (FlatBuffers)**: The C++ Service delegates the call via gRPC to the **Rust Service** using **FlatBuffers**.
> 4. **Rust to Go (Protobuf)**: The Rust Service passes the request via gRPC to the **Go-Service** using **Protobuf**.
> 5. **Data Persistence**: The Go-Service fetches/persists data using SQL in **MariaDB**.
> 6. **Return Path**: The response travels backward along the chain:<br>
>   `MariaDB` ➔ `Go-Service` (Protobuf) ➔ `Rust Service` (FlatBuffers) ➔ `C++ Service` (Protobuf) ➔ `Go HTTP Server` ➔ `Client Browser (AJAX)`.