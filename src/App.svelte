<script context="module" lang="ts">
  const themes = [
    "cerulean",
    "cosmo",
    "cyborg",
    "darkly",
    "flatly",
    "journal",
    "litera",
    "lumen",
    "lux",
    "materia",
    "minty",
    "morph",
    "pulse",
    "quartz",
    "sandstone",
    "simplex",
    "sketchy",
    "slate",
    "solar",
    "spacelab",
    "superhero",
    "united",
    "vapor",
    "yeti",
    "zephyr",
  ];
  const theme = themes[~~(Math.random() * themes.length)];
</script>

<script lang="ts">
  // import "./assets/bootswatch/quartz.min.css";
  import { onMount } from "svelte";
  import "bootstrap-icons/font/bootstrap-icons.css";
  import "./styles/styles.css";
  import { Source, Tauri } from "./services";
  import { Header, Body, Footer } from "./lib/layout";

  onMount(async () => {
    return Tauri.window.listen("tauri://focus", () => Source.refresh());
  });
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="/bootswatch/{theme}.min.css"
    type="text/css"
    media="screen"
  />
</svelte:head>

<main class="d-flex flex-column min-vh-100">
  <Header />
  <Body />
  <Footer />
</main>
