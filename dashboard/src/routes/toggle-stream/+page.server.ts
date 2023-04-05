import type { PageServerLoad, Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
  const res = await fetch('http://localhost:3000/get_streams').then(async (response) => {
    return await response.json();
  });
  const activeStreamers = res.names;

  if (
    activeStreamers.filter((streamer) => streamer.username === cookies.get('loggedIn')).length > 0
  ) {
    return { active: true };
  } else {
    return { active: false };
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, fetch, cookies }) => {
    const data = await request.formData();
    const token = data.get('token');

    if (token === null) {
      return fail(400, { incorrect: true });
    }

    const res = await fetch('http://localhost:3000/toggle_stream', {
      method: 'POST',
      headers: {
        'Content-type': 'application/json'
      },
      body: JSON.stringify({
        api_token: token,
        org_id: cookies.get('loggedIn')
      })
    })
      .then((response) => {
        if (response.ok) {
          return response.json();
        }
        throw new Error('Response not ok');
      })
      .catch(() => {
        return fail(400, { incorrect: true });
      });

    if (res.error !== null) {
      return fail(400, { incorrect: true });
    }

    redirect(303, '/toggle-stream');
  }
} satisfies Actions;
