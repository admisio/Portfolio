import { join as pathJoin } from "https://deno.land/std@0.183.0/path/mod.ts";
import { parse as parseCsv } from "https://deno.land/std@0.183.0/csv/parse.ts";
import { copy } from "https://deno.land/std@0.183.0/fs/copy.ts";

// AGE private key
const keyPath = await Deno.realPath("key.txt");
const key = await Deno.readTextFile(keyPath);

// Application table csv
const applicationList = parseCsv(await Deno.readTextFile("application.csv"), {
  skipFirstRow: true,
  separator: ",",
});

// Candidate table csv
const candidateList = parseCsv(await Deno.readTextFile("candidate.csv"), {
  skipFirstRow: true,
  separator: ",",
});

// portfolio-cli must be in PATH
const decodePortfolioAgeString = async (encrypted: string) => {
  const decrypt = Deno.run({
    cmd: ["portfolio-cli", "asymmetric", "-d", "-k", key, "-i", encrypted],
    stdout: "piped",
  });

  const output = new TextDecoder().decode(await decrypt.output());

  decrypt.close();

  return output.trim();
};

const DIR = "named";

// Data contains folders with the application id as name
for await (const folder of Deno.readDir("data")) {
  if (!folder.isDirectory) {
    console.error("Not a directory");
    console.error(folder.name);
    Deno.exit(1);
  }

  const applicationId = folder.name;

  const filteredApplications = applicationList.filter(
    (application) => application.candidate_id === applicationId
  );

  const application = filteredApplications[0];

  if (application == undefined) {
    console.error("Application not found");
    console.error(applicationId);
    //console.error(application);
    Deno.exit(1);
  }

  const candidateId = application.candidate_id;

  const candidate = candidateList.find(
    (candidate) => candidate.id === candidateId
  );

  const candidateNameEncoded = candidate!.name as string;
  const candidateSurnameEncoded = candidate!.surname as string;

  if (
    !candidateNameEncoded ||
    candidateNameEncoded === "" ||
    !candidateSurnameEncoded ||
    candidateSurnameEncoded === ""
  ) {
    // Neodevzdali dotazník
    console.error("Candidate name or surname undefined");
    console.error(candidateId);
    //console.error(candidate);
    //Deno.exit(1);
    continue;
  }

  const name = await decodePortfolioAgeString(candidateNameEncoded);
  const surname = await decodePortfolioAgeString(candidateSurnameEncoded);
  const code = application.id;

  let newFolderName = `${name}${surname}_${code}`;

  if (filteredApplications.length == 2)
    newFolderName += `_${filteredApplications[1].id}`;

  await copy(pathJoin("data", folder.name), pathJoin(DIR, newFolderName), {
    overwrite: true,
  });
}
