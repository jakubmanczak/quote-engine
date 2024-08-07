const qfetch = async (
  path: string | URL | globalThis.Request,
  init?: RequestInit
) => {
  const input =
    process.env.NODE_ENV === "production"
      ? `${process.env.NEXT_PUBLIC_SERVER_PATH}${path}`
      : `http://localhost:2019${path}`;
  return fetch(input, {
    credentials: "include",
    ...init,
  });
};

export { qfetch };
