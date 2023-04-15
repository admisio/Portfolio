import { join as pathJoin } from "https://deno.land/std@0.183.0/path/mod.ts";
import { exists } from "https://deno.land/std@0.183.0/fs/mod.ts";

const keyPath = await Deno.realPath("key.txt");

const DIR = "named";

for await (const folder of Deno.readDir(DIR)) {
  const ageFile = pathJoin(DIR, folder.name, "PORTFOLIO.age");
  const outFile = pathJoin(DIR, folder.name, "PORTFOLIO.zip");

  const fileExists = await exists(ageFile, { isFile: true });

  if (fileExists) {
    // rage must be in PATH
    const decrypt = Deno.run({
      cmd: ["rage", "-d", "-i", keyPath, ageFile, "-o", outFile],
      stdout: "piped",
    });
    await decrypt.output();
    decrypt.close();
  }
}
