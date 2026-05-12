import { createPost, listPosts, type BbsPost } from "./api.js";
import { term } from "./term.js";

/** Read-only/write BBS browser used by the Clubhouse. */
export async function runBbsUI(token: string): Promise<void> {
  term.clear();
  term.bold.cyan("◇ CLUBHOUSE BBS ◇\n");
  term.gray("Press N to compose new post, ESC to leave.\n\n");

  await refresh();

  return new Promise<void>((resolve) => {
    const handler = async (key: string) => {
      if (key === "ESCAPE") {
        term.removeListener("key", handler);
        resolve();
        return;
      }
      if (key === "n" || key === "N") {
        term("Title: ");
        const title = await readLine();
        term("Body : ");
        const body = await readLine();
        try {
          await createPost(token, title, body);
          term.green("✓ Posted.\n\n");
          await refresh();
        } catch (e) {
          term.red(`✘ ${(e as Error).message}\n`);
        }
      }
    };
    term.on("key", handler);
  });
}

async function refresh() {
  try {
    const posts = await listPosts();
    if (posts.length === 0) {
      term.gray("(no posts yet — press N to write one)\n");
      return;
    }
    for (const p of posts.slice(0, 20)) renderPost(p);
  } catch (e) {
    term.red(`✘ ${(e as Error).message}\n`);
  }
}

function renderPost(p: BbsPost) {
  const when = new Date(p.at).toLocaleString();
  term.bold.white(`▸ ${p.title}\n`);
  term.gray(`  by ${p.author} · ${when}\n`);
  term(`  ${p.body}\n\n`);
}

function readLine(): Promise<string> {
  return new Promise((resolve) => {
    term.inputField({}, (_e: Error | undefined, raw: string | undefined) => {
      term("\n");
      resolve((raw ?? "").trim());
    });
  });
}
