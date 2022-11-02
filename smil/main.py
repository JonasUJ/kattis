import re
smil = re.compile(r":\)|;\)|:-\)|;-\)")
print("\n".join(str(m.start()) for m in smil.finditer(input())))
