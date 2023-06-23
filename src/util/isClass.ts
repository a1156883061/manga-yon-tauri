export function isClass(type: string, obj: unknown) {
  const className = Object.getPrototypeOf(obj).constructor.name;
  console.log('clazz', className);
  return obj !== undefined && obj !== null && className === type;
}
