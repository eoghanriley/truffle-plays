"use strict";

const queue = [];

module.exports = async function(fastify, opts) {
  fastify.get("/*", async function(request, reply) {
    if (request.headers.auth === process.env.AUTH) {
      if (queue.length > 0) {
        return { input: queue.shift() };
      } else {
        return { input: "waiting" };
      }
    } else {
      return { input: "auth_error" };
    }
  });

  fastify.post("/*", async function(request, reply) {
    queue.push(request.body.input);
    reply
      .code(200)
      .header("Access-Control-Allow-Origin", "*")
      .header("Access-Control-Allow-Methods", "GET,HEAD,PUT,PATCH,POST,DELETE")
      .header(
        "Access-Control-Allow-Headers",
        "Origin, X-Requested-With, Content-Type, Accept"
      )
      .send({ status: "success" });
  });

  fastify.options("/*", async function(request, reply) {
    reply
      .code(204)
      .header("Content-Length", "0")
      .header("Access-Control-Allow-Origin", "*")
      .header("Access-Control-Allow-Methods", "GET,HEAD,PUT,PATCH,POST,DELETE")
      .header(
        "Access-Control-Allow-Headers",
        "Origin, X-Requested-With, Content-Type, Accept"
      )
      .send();
  });
};
