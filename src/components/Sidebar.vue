<script setup lang="ts">
interface MenuItem {
  title: string;
  value: string;
  icon: string;
}

interface Props {
  selectedItem: string;
  menuItems: MenuItem[];
}

interface Emits {
  (e: 'select-item', item: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const selectMenuItem = (item: string) => {
  emit('select-item', item);
};
</script>

<template>
  <v-navigation-drawer permanent width="200">
    <v-list>
      <v-list-item
        v-for="item in menuItems"
        :key="item.value"
        :prepend-icon="item.icon"
        :title="item.title"
        :active="selectedItem === item.value"
        :color="selectedItem === item.value ? 'green' : undefined"
        @click="selectMenuItem(item.value)"
        class="mb-1"
      ></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>
/* Sidebar styles */
</style>
