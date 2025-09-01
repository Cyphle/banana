import Fastify from 'fastify';

const fastify = Fastify({ logger: true });

const HOST = process.env.HOST || '0.0.0.0';
const PORT = parseInt(process.env.PORT || '8080', 10);

fastify.get('/', async () => {
  return 'debug-app (Node.js + Fastify) is running';
});

fastify.get('/healthz', async (request, reply) => {
  return reply.code(200).send();
});

const start = async () => {
  try {
    await fastify.listen({ port: PORT, host: HOST });
    fastify.log.info(`Server listening on http://${HOST}:${PORT}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
