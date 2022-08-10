<script lang="ts">
  import { onMount } from "svelte";
  import "bootstrap-icons/font/bootstrap-icons.css";
  import "./styles/styles.css";
  import { Source, Tauri } from "./services";
  import { Header, Body, Footer } from "./lib/layout";

  onMount(Source.refreshOnFocusListener);
</script>

<svelte:head>
  <link rel="stylesheet" href="/bootstrap.min.css">
</svelte:head>

<main class="d-flex flex-column min-vh-100">
  <Header />
  <Body />
  <Footer />
</main>
