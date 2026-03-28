// @ts-nocheck
function __floeEq(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (a == null || b == null) return false;
  if (typeof a !== "object" || typeof b !== "object") return false;
  const ka = Object.keys(a as object);
  const kb = Object.keys(b as object);
  if (ka.length !== kb.length) return false;
  return ka.every((k) => __floeEq((a as Record<string, unknown>)[k], (b as Record<string, unknown>)[k]));
}

import { useState, Suspense } from "react";

import { useSuspenseQuery, QueryClient, QueryClientProvider, QueryErrorResetBoundary } from "@tanstack/react-query";

import { ErrorBoundary } from "react-error-boundary";

type Post = { id: number; title: string; body: string; userId: number };

type User = { id: number; name: string; email: string; company: { name: string } };

const queryClient = new QueryClient({ defaultOptions: { queries: { staleTime: 60_000, retry: 1 } } });

async function fetchUser(userId: number): { ok: true; value: User } | { ok: false; error: Error } {
  return (() => { const __v = (() => { const __r = (async () => { try { const _r = await await (() => { const __r = (async () => { try { const _r = await fetch(`https://jsonplaceholder.typicode.com/users/${userId}`); return { ok: true as const, value: _r }; } catch (_e) { return { ok: false as const, error: _e instanceof Error ? _e : new Error(String(_e)) }; } })(); if (typeof __r === 'object' && __r !== null && 'ok' in __r && typeof __r.ok === 'boolean') { if (!__r.ok) throw __r.error; return __r.value; } return __r; })().json(); return { ok: true as const, value: _r }; } catch (_e) { return { ok: false as const, error: _e instanceof Error ? _e : new Error(String(_e)) }; } })(); if (typeof __r === 'object' && __r !== null && 'ok' in __r && typeof __r.ok === 'boolean') { if (!__r.ok) throw __r.error; return __r.value; } return __r; })(); if (typeof __v !== "object" || __v === null) return { ok: false as const, error: new Error("expected object, got " + typeof __v) }; return { ok: true as const, value: __v as User }; })();
}

async function fetchPosts(url: string): { ok: true; value: Array<Post> } | { ok: false; error: Error } {
  return (() => { const __v = (() => { const __r = (async () => { try { const _r = await await (() => { const __r = (async () => { try { const _r = await fetch(url); return { ok: true as const, value: _r }; } catch (_e) { return { ok: false as const, error: _e instanceof Error ? _e : new Error(String(_e)) }; } })(); if (typeof __r === 'object' && __r !== null && 'ok' in __r && typeof __r.ok === 'boolean') { if (!__r.ok) throw __r.error; return __r.value; } return __r; })().json(); return { ok: true as const, value: _r }; } catch (_e) { return { ok: false as const, error: _e instanceof Error ? _e : new Error(String(_e)) }; } })(); if (typeof __r === 'object' && __r !== null && 'ok' in __r && typeof __r.ok === 'boolean') { if (!__r.ok) throw __r.error; return __r.value; } return __r; })(); if (!Array.isArray(__v)) return { ok: false as const, error: new Error("expected array, got " + typeof __v) }; for (let __i3 = 0; __i3 < __v.length; __i3++) { if (typeof __v[__i3] !== "object" || __v[__i3] === null) return { ok: false as const, error: new Error("element [" + __i3 + "]: expected object, got " + typeof __v[__i3]) }; } return { ok: true as const, value: __v as Array<Post> }; })();
}

function PostAuthor(props: { userId: number }): JSX.Element {
  const { data } = useSuspenseQuery({ queryKey: ["user", props.userId], queryFn: async () => fetchUser(props.userId) });
  return <span className={"text-indigo-400"}>{data.ok === true ? (() => { const user = data.value; return `${user.name} (${user.company.name})`; })() : data.ok === false ? (() => { const e = data.error; return `Error: ${e.message}`; })() : (() => { throw new Error("non-exhaustive match"); })()}</span>;
}

function PostList(): JSX.Element {
  const [selectedUserId, setSelectedUserId] = useState(undefined);
  const url = selectedUserId.tag === "Some" ? (() => { const uid = selectedUserId.value; return `https://jsonplaceholder.typicode.com/posts?userId=${uid}`; })() : selectedUserId.tag === "None" ? "https://jsonplaceholder.typicode.com/posts?_limit=10" : (() => { throw new Error("non-exhaustive match"); })();
  const { data } = useSuspenseQuery({ queryKey: ["posts", selectedUserId], queryFn: async () => fetchPosts(url) });
  const posts = data.ok === true ? (() => { const p = data.value; return p; })() : data.ok === false ? [] : (() => { throw new Error("non-exhaustive match"); })();
  const userIds = [1, 2, 3, 4, 5];
  return <div><div className={"mb-6 flex flex-wrap gap-2"}><button onClick={() => setSelectedUserId(undefined)} className={selectedUserId.tag === "None" ? "rounded px-3 py-1 text-sm bg-indigo-600 text-white transition-colors" : "rounded px-3 py-1 text-sm bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition-colors"}>All</button>{userIds.map((id) => <button key={id} onClick={() => setSelectedUserId(id)} className={selectedUserId.tag === "Some" ? (() => { const selected = selectedUserId.value; if (__floeEq(selected, id)) { return "rounded px-3 py-1 text-sm bg-indigo-600 text-white transition-colors"; } return "rounded px-3 py-1 text-sm bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition-colors"; })() : "rounded px-3 py-1 text-sm bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition-colors"}>{`User ${id}`}</button>)}</div><div className={"space-y-4"}>{posts.map((post) => <article key={post.id} className={"rounded-lg border border-zinc-800 bg-zinc-900/50 p-5"}><h3 className={"mb-1 text-lg font-semibold capitalize text-zinc-100"}>{post.title}</h3><Suspense fallback={<span className={"text-sm text-zinc-600"}>Loading author...</span>}><p className={"mb-3 text-sm"}>{"by "}<PostAuthor userId={post.userId} /></p></Suspense><p className={"text-sm leading-relaxed text-zinc-400"}>{post.body}</p></article>)}</div></div>;
}

function LoadingSkeleton(): JSX.Element {
  const skeletons = [1, 2, 3];
  return <div className={"space-y-4"}>{skeletons.map((i) => <div key={i} className={"animate-pulse rounded-lg border border-zinc-800 bg-zinc-900/50 p-5"}><div className={"mb-2 h-5 w-3/4 rounded bg-zinc-800"} /><div className={"mb-3 h-4 w-1/4 rounded bg-zinc-800"} /><div className={"space-y-2"}><div className={"h-3 w-full rounded bg-zinc-800"} /><div className={"h-3 w-5/6 rounded bg-zinc-800"} /></div></div>)}</div>;
}

export function PostsPage(): JSX.Element {
  return <QueryClientProvider client={queryClient}><h1 className={"mb-2 text-3xl font-bold"}>Posts</h1><p className={"mb-6 text-zinc-400"}>TanStack Query + Suspense demo using JSONPlaceholder API.</p><QueryErrorResetBoundary>{({ reset }) => <ErrorBoundary onReset={reset} fallbackRender={({ resetErrorBoundary, error }) => <div className={"rounded-lg border border-red-900/50 bg-red-950/30 p-6 text-center"}><p className={"mb-3 text-red-400"}>{`Failed to load posts: ${error.message}`}</p><button onClick={resetErrorBoundary} className={"rounded bg-red-600 px-4 py-2 text-sm text-white hover:bg-red-500"}>Retry</button></div>}><Suspense fallback={<LoadingSkeleton />}><PostList /></Suspense></ErrorBoundary>}</QueryErrorResetBoundary></QueryClientProvider>;
}
