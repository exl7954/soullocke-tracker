import { useState, useEffect, useRef } from "react";
import { FileButton, Button, Group, Text } from "@mantine/core";
import { readFile } from "../util/uploadFile";
import { mapPokemon } from "../util/mapPokemon";

export default function SaveUpload() {
  const [file, setFile] = useState<File | null>(null);
  const [trainerId, setTrainerId] = useState<string>("");
  const [expected, setExpected] = useState<string>("");

  const resetRef = useRef<() => void>(null);
  const clearFile = () => {
    setFile(null);
    resetRef.current?.();
  };

  useEffect(() => {
    if (file) {
      (async () => {
        const result = await readFile(file);
        setTrainerId(result?.data?.trainedId.toString() ?? "");
        if (result?.data?.trainedId) {
          let poke1 = mapPokemon(result?.data?.trainedId, "Chikorita");
          let poke2 = mapPokemon(result?.data?.trainedId, "Cyndaquil");
          let poke3 = mapPokemon(result?.data?.trainedId, "Totodile");

          setExpected(poke1 + ", " + poke2 + ", " + poke3);
        }
      })();
    } else {
      setTrainerId("");
    }
  }, [file]);

  return (
    <>
      <Group justify="center">
        <FileButton resetRef={resetRef} onChange={setFile} accept="">
          {(props) => <Button {...props}>Upload .sav</Button>}
        </FileButton>
        <Button disabled={!file} color="red" onClick={clearFile}>
          Reset
        </Button>
      </Group>

      {file && (
        <>
          <Text size="sm" ta="center" mt="sm">
            trainer id: {trainerId}
          </Text>
          <Text>{expected}</Text>
        </>
      )}
    </>
  );
}
