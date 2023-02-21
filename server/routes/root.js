"use strict";

const queue = [];

module.exports = async function(fastify, opts) {
  fastify.get("/", async function(request, reply) {
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

  fastify.post("/", async function(request, reply) {
    queue.push(request.body.input);
  });
};
