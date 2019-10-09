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
    import('../../pkg').then(async wasm => {
      const result = await wasm.run();
      console.log(result);
    });
  }
}
</script>

<style scoped lang="scss">
</style>
