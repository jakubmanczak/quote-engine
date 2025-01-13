export function GrainEffect() {
  return (
    <svg
      className="absolute top-0 left-0 z-0 isolate h-full w-full opacity-[0.10] select-none pointer-events-none"
      aria-hidden
    >
      <filter id="grain">
        <feTurbulence
          type="turbulence"
          baseFrequency="0.65"
          numOctaves="3"
          stitchTiles="stitch"
        />
      </filter>
      <rect width="100%" height="100%" filter="url(#grain)" />
    </svg>
  );
}
