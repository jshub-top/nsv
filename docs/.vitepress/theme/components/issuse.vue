<script setup lang="ts" >

export interface Props {
    size: number
    path?: string
    title?: string
}

const props = withDefaults(defineProps<Props>(), {
    size: () => ""
})

</script>
<template>

</template>
<style lang="scss" scoped >
</style>