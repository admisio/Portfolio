import { join as pathJoin } from "https://deno.land/std@0.183.0/path/mod.ts";
import { exists } from "https://deno.land/std@0.183.0/fs/mod.ts";

const DIR = "named";

for await (const folder of Deno.readDir(DIR)) {
  const ageFile = pathJoin(DIR, folder.name, "PORTFOLIO.age");
  const fileExists = await exists(ageFile);

  if (fileExists) {
    await Deno.remove(ageFile);
  }
}
