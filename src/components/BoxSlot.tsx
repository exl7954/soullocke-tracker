import { Box } from "@mantine/core";

interface ComponentProps {
  pokemon1: string | null;
  pokemon2: string | null;
}

export default function BoxSlot(props: ComponentProps) {
  return (
    <Box>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="-0.5 -0.5 16 16"
        fill="none"
        stroke="#8e8e8e"
        stroke-linecap="round"
        stroke-linejoin="round"
        id="Pokeball--Streamline-Tabler"
        height="100"
        width="100"
        transform="rotate(-45)"
      >
        <desc>Pokeball Streamline Icon: https://streamlinehq.com</desc>
        <path
          d="M1.875 7.5a5.625 5.625 0 1 0 11.25 0 5.625 5.625 0 1 0 -11.25 0"
          stroke-width="1"
        ></path>
        <path
          d="M5.625 7.5a1.875 1.875 0 1 0 3.75 0 1.875 1.875 0 1 0 -3.75 0"
          stroke-width="1"
        ></path>
        <path d="M1.875 7.5h3.75" stroke-width="1"></path>
        <path d="M9.375 7.5h3.75" stroke-width="1"></path>
      </svg>
    </Box>
  );
}
