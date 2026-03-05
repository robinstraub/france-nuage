import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { userManager } from "../auth/user-manager";
import { config } from "../config";

export async function getAuthenticatedTransport() {
  const user = await userManager.getUser();
  const meta: Record<string, string> = {};
  if (user?.access_token) {
    meta.authorization = `Bearer ${user.access_token}`;
  }
  return new GrpcWebFetchTransport({
    baseUrl: config.api.baseUrl,
    meta,
  });
}
