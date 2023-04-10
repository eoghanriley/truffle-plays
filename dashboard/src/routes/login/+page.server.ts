import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
  default: async ({ request, fetch, cookies }) => {
    const data = await request.formData();
    const orgId = data.get('orgId');
    const password = data.get('password');

    if (orgId === null || password === null) {
      return fail(400, { incorrect: true });
    }

    const res = await fetch('http://localhost:3000/login', {
      method: 'POST',
      headers: {
        'Content-type': 'application/json'
      },
      body: JSON.stringify({
        org_id: orgId,
        password: password
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

    cookies.set('loggedIn', orgId?.toString());

    // Changes page so that token isn't in url for security
    throw redirect(303, '/');
  }
} satisfies Actions;
