<script setup lang="ts">
import { reactive, watch } from "vue";

export interface Instruction {
  address: number | "";
  label: string;
  opcode: string;
  operand: string;
}

const props = defineProps<{
  modelValue: Instruction;
  address: number;
  addressMin: number;
  flagged: boolean;
}>();
const emit = defineEmits(["update:modelValue", "update:address"]);

const instruction = reactive<Instruction>({ ...props.modelValue });
watch(instruction, () => emit("update:modelValue", instruction));
</script>

<template>
  <div>
    <input
      :placeholder="`${address}`"
      :style="{ backgroundColor: flagged ? 'pink' : undefined }"
      type="number"
      :min="addressMin"
      v-model="instruction.address"
    />
    <input placeholder="Label" v-model="instruction.label" />
    <input placeholder="Opcode" v-model="instruction.opcode" />
    <input placeholder="Operand" v-model="instruction.operand" />
  </div>
</template>
