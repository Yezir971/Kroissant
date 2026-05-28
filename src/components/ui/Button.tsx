type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: 'primary' | 'secondary' | 'inverted' | 'outlined';
  size?: 'sm' | 'md' | 'lg';
};

export const Button = ({ 
  children, 
  variant = 'primary', 
  size = 'md',
  className = '', 
  ...props 
}: ButtonProps) => {
  
  // Mapping strict vers les classes DaisyUI
  const variantClasses = {
    primary: "btn-primary",
    secondary: "btn-secondary",
    inverted: "btn-neutral",
    outlined: "btn-outline",
  };

  const sizeClasses = {
    sm: "btn-sm",
    md: "",
    lg: "btn-lg",
  };

  return (
    <button 
      className={`btn ${variantClasses[variant]} ${sizeClasses[size]} rounded-md font-bold ${className}`} 
      {...props}
    >
      {children}
    </button>
  );
};