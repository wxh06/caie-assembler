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
  start: number;
  flagged: boolean;
}>();
const emit = defineEmits<{
  (e: "update:modelValue", instruction: Instruction): void;
  (e: "update:start", address: number): void;
  (e: "focus"): void;
}>();

const instruction = reactive<Instruction>({ ...props.modelValue });
watch(instruction, () => emit("update:modelValue", instruction));
</script>

<script lang="ts">
export const notEmpty = (instruction: Instruction) =>
  Object.values(instruction).map(Boolean).includes(true);
</script>

<template>
  <tr>
    <td>
      <input
        class="form-check-input align-middle"
        type="radio"
        name="start"
        v-if="notEmpty(instruction)"
        :checked="address === start"
        @input="$emit('update:start', address)"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        :placeholder="`${address}`"
        :class="{ 'is-invalid': flagged }"
        type="number"
        :min="addressMin"
        v-model="instruction.address"
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        v-model="instruction.label"
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        v-model="instruction.opcode"
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        v-model="instruction.operand"
        @focus="emit('focus')"
      />
    </td>
  </tr>
</template>
