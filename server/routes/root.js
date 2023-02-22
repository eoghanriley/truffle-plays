"use strict";

const queue = [];

module.exports = async function(fastify, opts) {
  await fastify.register(import("@fastify/rate-limit"), {
    global: true,
    max: 1,
    timeWindow: 1000,
  });

  fastify.get(
    "/*",
    {
      config: {
        rateLimit: false,
      },
    },
    async function(request, reply) {
      reply
        .code(200)
        .header("Access-Control-Allow-Origin", "*")
        .header(
          "Access-Control-Allow-Methods",
          "GET,HEAD,PUT,PATCH,POST,DELETE"
        )
        .header(
          "Access-Control-Allow-Headers",
          "Origin, X-Requested-With, Content-Type, Accept"
        );

      if (request.headers.auth === process.env.AUTH) {
        if (queue.length > 0) {
          reply.send({ input: queue.shift() });
        } else {
          reply.send({ input: "waiting" });
        }
      } else {
        reply.send({ input: "auth_error" });
      }
    }
  );

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
