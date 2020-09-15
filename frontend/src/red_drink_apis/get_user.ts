export type User = {
  id: number;
  name: string;
  avatar_url: string | null;
  email: string | null;
  created_at: string;
};

export const getUser = (): Promise<User> =>
  fetch('/api/user', { method: 'GET' }).then((response) => response.json());
