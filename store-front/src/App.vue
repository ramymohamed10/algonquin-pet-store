<template>
  <div id="app">
    <OrderForm />
  </div>
</template>

<script>
import OrderForm from './components/OrderForm.vue'
export default {
  name: 'App',
  components: {
    OrderForm
  }
}
</script>

<style>
body {
  background-image: url('@/assets/algonquin.jpg');
  background-size: cover;
  background-position: center;
  background-attachment: fixed; /* Keeps the background in place when scrolling */
  margin: 0;
  padding: 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
