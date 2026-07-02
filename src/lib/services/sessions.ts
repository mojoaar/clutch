import { PROVIDERS } from "./providers";
import { getSetting } from "./settings";
import * as db from "$lib/db";
import { chatStore, type Session } from "$lib/stores/chat";

export async function createNewSession(title: string): Promise<Session> {
  const id = crypto.randomUUID();
  const defaultProviderId =
    (await getSetting("default_provider")) || "deepseek";
  const provider = PROVIDERS[defaultProviderId] || PROVIDERS.deepseek;
  const savedModel = await getSetting(`default_model_${provider.id}`);
  const model = savedModel || provider.defaultModel;

  const session: Session = {
    id,
    title,
    provider: provider.id,
    model,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  await db.createSession(id, session.title, session.provider, session.model);
  chatStore.addSession(session);
  chatStore.setActiveSession(id);

  return session;
}
