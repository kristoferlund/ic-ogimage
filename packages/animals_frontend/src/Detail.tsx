import NotFound from "./NotFound";
import { animals_canister } from "./declarations";
import { useQuery } from "@tanstack/react-query";

async function getAnimal(id: number) {
  return animals_canister.animals_get(id);
}

export default function Detail({ id }: { id: number }) {
  const {
    data: animal,
    isPending,
    isError,
  } = useQuery({ queryKey: ["animals", id], queryFn: () => getAnimal(id) });

  return (
    <section className="column section-bg">
      <a href="/">← Back</a>

      {isPending && <p>Loading...</p>}

      {isError && <NotFound />}

      {animal && (
        <div>
          <h1>{animal.name}</h1>

          <p>{animal.emoji}</p>

          <p>OG Image:</p>

          <img src={`/${id}/ogimage.png`} width="100%" />
        </div>
      )}
    </section>
  );
}
