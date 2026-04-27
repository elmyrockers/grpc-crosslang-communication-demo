# gRPC-CrossLang-Communication-Demo
Minimal cross-language gRPC demo illustrating service-to-service communication between C++, Rust, and Go using FlatBuffers payloads.

## Tech Stack

<!-- ![Go](https://img.shields.io/badge/Go-00ADD8?style=for-the-badge&logo=go&logoColor=white)
![Fiber](https://img.shields.io/badge/Fiber-00ACD7?style=for-the-badge)
![gRPC](https://img.shields.io/badge/gRPC-00A1E0?style=for-the-badge)
![Protobuf](https://img.shields.io/badge/Protobuf-336791?style=for-the-badge)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-FF6F00?style=for-the-badge)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-38B2AC?style=for-the-badge)
![Alpine.js](https://img.shields.io/badge/Alpine.js-8BC0D0?style=for-the-badge)
![Jet](https://img.shields.io/badge/Jet-Template-blue?style=for-the-badge) -->


<!-- ![Go](https://img.shields.io/badge/Go-Backend-blue?style=flat-square&logo=go&logoColor=white)
![Fiber](https://img.shields.io/badge/Fiber-HTTP-lightgrey?style=flat-square)
![gRPC](https://img.shields.io/badge/gRPC-RPC-blueviolet?style=flat-square&logo=grpc&logoColor=white)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-Serialization-orange?style=flat-square)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-UI-38B2AC?style=flat-square&logo=tailwind-css&logoColor=white)
![Alpine.js](https://img.shields.io/badge/Alpine.js-Interactivity-8BC0D0?style=flat-square)
![Jet](https://img.shields.io/badge/Jet-Template-grey?style=flat-square) -->


<!-- ![Go](https://img.shields.io/badge/Go-Backend-0F172A?style=flat-square)
![Fiber](https://img.shields.io/badge/Fiber-HTTP-0F172A?style=flat-square)
![gRPC](https://img.shields.io/badge/gRPC-RPC-0F172A?style=flat-square)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-Serialization-0F172A?style=flat-square)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-Frontend-0F172A?style=flat-square)
![Alpine.js](https://img.shields.io/badge/Alpine-UI-0F172A?style=flat-square)
![Jet](https://img.shields.io/badge/Jet-Template-0F172A?style=flat-square) -->

<!-- ![Go](https://img.shields.io/badge/Go-Backend-00ADD8?style=flat-square&logo=go)
![Fiber](https://img.shields.io/badge/Fiber-HTTP-2C8EBB?style=flat-square&logo=fiber)
![gRPC](https://img.shields.io/badge/gRPC-RPC-4285F4?style=flat-square&logo=grpc)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-Serialization-005F9E?style=flat-square)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-Frontend-38B2AC?style=flat-square&logo=tailwind-css)
![Alpine.js](https://img.shields.io/badge/Alpine-UI-77C1D2?style=flat-square&logo=alpine.js)
![Jet](https://img.shields.io/badge/Jet-Template-FF5A5F?style=flat-square) -->


<!-- ![Go](https://img.shields.io/badge/Go-Backend-00ADD8?style=flat-square&logo=go)
![C++](https://img.shields.io/badge/C++-Server-00599C?style=flat-square&logo=cplusplus)
![Rust](https://img.shields.io/badge/Rust-Server-000000?style=flat-square&logo=rust)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-Frontend-38B2AC?style=flat-square&logo=tailwind-css)
![Alpine.js](https://img.shields.io/badge/Alpine-UI-77C1D2?style=flat-square&logo=alpine.js)
![Axios](https://img.shields.io/badge/Axios-HTTP%20Client-5A29E4?style=flat-square&logo=axios)
![Docker](https://img.shields.io/badge/Docker-Container-2496ED?style=flat-square&logo=docker)

![Fiber](https://img.shields.io/badge/Fiber-HTTP-00ACD7?style=flat)
![gRPC](https://img.shields.io/badge/gRPC-RPC-4285F4?style=flat-square)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-Serialization-005F9E?style=plastic) -->

<!-- ![Go](https://img.shields.io/badge/Go-Backend-lightgrey?style=flat-square)
![Fiber](https://img.shields.io/badge/Fiber-HTTP-lightgrey?style=flat-square)
![gRPC](https://img.shields.io/badge/gRPC-RPC-lightgrey?style=flat-square)
![FlatBuffers](https://img.shields.io/badge/FlatBuffers-Serialization-lightgrey?style=flat-square)
![Jet](https://img.shields.io/badge/Jet-Template-lightgrey?style=flat-square)
![C++](https://img.shields.io/badge/C++-Server-lightgrey?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-Server-lightgrey?style=flat-square)
![Axios](https://img.shields.io/badge/Axios-HTTP%20Client-lightgrey?style=flat-square)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-Frontend-lightgrey?style=flat-square)
![Alpine.js](https://img.shields.io/badge/Alpine-UI-lightgrey?style=flat-square)
![Docker](https://img.shields.io/badge/Docker-Container-lightgrey?style=flat-square)
 -->

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
