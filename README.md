# The Lmao Programming Language - The Spiritual Successor of EcksDee
## By Jesse A. Jones (KesseNones)

# <a name = "toc"></a> Table of Contents
- [0 Introduction](#intro)
- [1 Using a Stack-Based Approach](#stack-based)
- [2 Data Types Used](#data-types)
- [3 Operators](#ops)
- [4 Fancy Operators](#fancy-ops)
- [5 Conclusion](#conclusion)


## <a name = "intro"></a>0 Introduction
Welcome to Lmao, the spiritual successor of the EcksDee programming language! This language was made using the general layout of EcksDee as a base but built in Rust instead of Haskell. This resulted in a notably faster language since Rust allows mutable state and has a lot of constant time data structures to take advantage of. Overall, this language aims to do what EcksDee did but better, with more sensible and consistent behavior on the stack with each operator and an overall greater performance.

### [**Return to Table of Contents**](#toc)

## <a name = "stack-based"></a>1 Using a Stack-Based Approach
Like EcksDee, Lmao operates in a similar manner to Forth, with a stack and operators for the stack. However, there is a greater consistency in behavior with the stack and how operators work on it. Combined with the stack is also a built in heap to improve efficiency for more complex data types.

The stack works like this: data types are pushed to the stack and operators consume those data and push new data to the stack after successfully completing the operation. 

For example:
```
2 3 +
```
This code pushes `isize 2` and `isize 3` to the stack and then uses the `+` operator which consumes the two values on the stack and pushes a new `isize 5` that is the result of adding the two values of compatible types together. 

What's an `isize`? What are operators? All in good time. The point is that the programming language works by using operators that take values pushed to the stack which then potentially produce new values.

### [**Return to Table of Contents**](#toc)

## <a name = "data-types"></a> 2 Data Types Used



Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.

### [**Return to Table of Contents**](#toc)

## <a name = "ops"></a> 3 Operators


Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.

### [**Return to Table of Contents**](#toc)

## <a name = "fancy-ops"></a> 4 Fancy Operators



Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.


### [**Return to Table of Contents**](#toc)

## <a name = "conclusion"></a> 5 Conclusion 




Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.


### [**Return to Table of Contents**](#toc)