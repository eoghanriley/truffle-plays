import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
  default: async ({ request, fetch, cookies, url }) => {
    const data = await request.formData();
    const orgId = data.get('orgId');
    const stream = data.get('stream');
    const password = data.get('password');
    const confirmPassword = data.get('confirmPassword');

    const key = url.toString().split('/signup/')[1];

    if (orgId === null || stream === null || password === null || confirmPassword === null) {
      return fail(400, { incorrect: true });
    }

    if (password !== confirmPassword) {
      return fail(400, { incorrect: true });
    }

    const res = await fetch(`http://localhost:3000/register/${key}`, {
      method: 'POST',
      headers: {
        'Content-type': 'application/json'
      },
      body: JSON.stringify({
        org_id: orgId,
        password: password,
        stream: stream
      })
    })
      .then((response) => {
        if (response.ok) {
          return response.json();
        }
        throw new Error('Response not ok');
      })
      .catch((error) => {
        console.log(error);
        return fail(400, { incorrect: true });
      });

    if (res.error !== null) {
      console.log(res.error);
      return fail(400, { key_incorrect: true });
    }

    cookies.set('loggedIn', orgId?.toString());
    throw redirect(303, '/');
  }
} satisfies Actions;
