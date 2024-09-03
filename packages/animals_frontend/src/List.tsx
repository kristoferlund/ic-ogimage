import { animals_canister } from "./declarations";
import { useQuery } from "@tanstack/react-query";

async function listAnimals() {
  return animals_canister.animals_list();
}

export default function List() {
  const { data: animals, isPending } = useQuery({
    queryKey: ["animals"],
    queryFn: listAnimals,
  });

  return (
    <section className="column section-bg">
      <h1>OG Image</h1>

        <p>
          Smart contract canisters on the Internet Computer can serve content
          over HTTPS. That means whole websites can be hosted on the IC. This
          project demonstrates how to{" "}
          <b>dynamically generate Open Graph images</b> for an IC project. It
          also shows how to <b>dynamically modify the HTML metadata</b> of each
          page to improve SEO.
        </p>

      <p>Animals:</p>

      {isPending && <p>Loading...</p>}

      {animals && (
        <ul>
          {animals?.map((animal) => (
            <li key={animal.id}>
              <a href={`/${animal.id}`}>{animal.name}</a>
            </li>
          ))}
        </ul>
      )}

      <p>OG Image:</p>

      <img src={`/ogimage.png`} width="100%" />
    </section>
  );
}
