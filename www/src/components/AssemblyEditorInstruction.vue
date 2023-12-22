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
const emit = defineEmits([
  "update:modelValue",
  "update:address",
  "update:start",
]);

const instruction = reactive<Instruction>({ ...props.modelValue });
watch(instruction, () => emit("update:modelValue", instruction));
</script>

<template>
  <tr>
    <td>
      <input
        class="form-check-input align-middle"
        type="radio"
        name="start"
        v-if="
          instruction.address ||
          instruction.label ||
          instruction.opcode ||
          instruction.operand
        "
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
      />
    </td>
    <td>
      <input class="form-control form-control-sm" v-model="instruction.label" />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        v-model="instruction.opcode"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        v-model="instruction.operand"
      />
    </td>
  </tr>
</template>
