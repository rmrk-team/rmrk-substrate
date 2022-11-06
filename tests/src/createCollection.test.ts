import { getApiConnection } from "./substrate/substrate-api";
import { createCollection } from "./util/tx";

describe("Integration test: create new collection", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  const alice = "//Alice";

  it("create NFT collection", async () => {
    await createCollection(api, 50, alice, "test-metadata", 42, "test-symbol");
  });

  it("create NFT collection without token limit", async () => {
    await createCollection(
      api,
      51,
      alice,
      "no-limit-metadata",
      null,
      "no-limit-symbol"
    );
  });

  after(() => {
    api.disconnect();
  });
});
