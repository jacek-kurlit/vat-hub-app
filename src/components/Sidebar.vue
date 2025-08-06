<script setup lang="ts">
import { ref } from 'vue';

interface MenuItem {
  title: string;
  value: string;
  icon: string;
  children?: MenuItem[];
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

const expandedGroups = ref<string[]>(['contractors']);

const selectMenuItem = (item: string) => {
  emit('select-item', item);
};

const isChildSelected = (item: MenuItem): boolean => {
  if (!item.children) return false;
  return item.children.some(child => child.value === item.value);
};
</script>

<template>
  <v-navigation-drawer permanent width="250">
    <v-list v-model:opened="expandedGroups">
      <template v-for="item in menuItems" :key="item.value">
        <!-- Items with children (groups) -->
        <v-list-group
          v-if="item.children"
          :value="item.value"
        >
          <template v-slot:activator="{ props }">
            <v-list-item
              v-bind="props"
              :prepend-icon="item.icon"
              :title="item.title"
              :active="isChildSelected(item)"
              :color="isChildSelected(item) ? 'green' : undefined"
            ></v-list-item>
          </template>

          <v-list-item
            v-for="child in item.children"
            :key="child.value"
            :prepend-icon="child.icon"
            :title="child.title"
            :active="selectedItem === child.value"
            :color="selectedItem === child.value ? 'green' : undefined"
            @click="selectMenuItem(child.value)"
          ></v-list-item>
        </v-list-group>

        <!-- Items without children (regular items) -->
        <v-list-item
          v-else
          :prepend-icon="item.icon"
          :title="item.title"
          :active="selectedItem === item.value"
          :color="selectedItem === item.value ? 'green' : undefined"
          @click="selectMenuItem(item.value)"
          class="mb-1"
        ></v-list-item>
      </template>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>
/* Sidebar styles */
</style>
