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

      <img src={`/ogimage.svg`} width="100%" />
    </section>
  );
}
