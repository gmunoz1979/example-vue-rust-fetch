<template>
  <div class="fetch">
  </div>
</template>

<script>
export default {
  name: 'Fetch',
  props: {
    msg: String
  },
  beforeCreate() {
    import('../../pkg').then(wasm => {
      wasm.test("Hello World!!!");
    });
  }
}
</script>

<style scoped lang="scss">
</style>
