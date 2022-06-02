<script lang="ts">
  import "bootstrap-icons/font/bootstrap-icons.css";
  import "./styles/styles.css";

  import { Header, Body, Footer } from "./lib/layout";
  import { Theme } from "./services";

</script>

<main class="d-flex flex-column min-vh-100">
  <Header />
  <Body />
  <Footer />
</main>

<svelte:head>
  <link
    rel="stylesheet"
    href="/src/assets/bootswatch/{$Theme}.min.css"
    type="text/css"
    media="screen"
  />
</svelte:head>
