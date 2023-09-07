/*
** Naive Order Statistic Binary Search Tree implementation
*/

#include <iostream>
#include <cassert>

namespace bst
{
template <typename T>
class OrderStatisticTree
{
	struct node
	{
		T data;
		node* left{nullptr};
		node* right{nullptr};
		uint64_t size{1};

	public:
		node(T _data)
			: data{_data}
		{
		}
	};

private:
	node* root{nullptr};

private:
	node* makeEmpty(node* aNode)
	{
		if(nullptr == aNode)
			return nullptr;

		{
			makeEmpty(aNode->left);
			makeEmpty(aNode->right);

			delete aNode;
		}

		return nullptr;
	}

	node* insert(T aData, node* aNode)
	{
		if(aNode == nullptr)
			aNode = new node(aData);
		else if(aData < aNode->data)
		{
			aNode->left = insert(aData, aNode->left);
			aNode->size += 1;
		}
		else if(aData > aNode->data)
		{
			aNode->right = insert(aData, aNode->right);
			aNode->size += 1;
		}

		return aNode;
	}

	std::tuple<node*, T> removeBy(uint64_t aIndex, node* aNode)
	{
		//display(aNode, true);

		if(nullptr == aNode)
			return std::make_tuple(nullptr, T());

		const auto p = aNode->left ? aNode->left->size + 1 : 1;

		if(aIndex < p)
		{
			auto [node, data] = removeBy(aIndex, aNode->left);

			aNode->left = node;
			aNode->size -= 1;

			return std::make_tuple(aNode, data);
		}
		else if(aIndex > p)
		{
			auto [node, data] = removeBy(aIndex - p, aNode->right);
			aNode->right = node;
			aNode->size -= 1;

			return std::make_tuple(aNode, data);
		}
		else if(p == aIndex)
		{
			if(aNode->left && aNode->right)
			{
				auto [node, data] = removeBy(1, aNode->right);
				aNode->right = node;
				aNode->data = data;
				aNode->size -= 1;
			}
			else
			{
				// single or none leaf
				std::unique_ptr<node> temp(aNode);
				aNode = nullptr;

				if(nullptr != temp->left)
					aNode = temp->left;
				else if(nullptr != temp->right)
					aNode = temp->right;

				return std::make_tuple(aNode, temp->data);
			}
		}

		return std::make_tuple(aNode, T());
	}

	void display(node const* const aNode, bool aNewLine = false) const
	{
		if(nullptr == aNode)
			std::cout << "null pointer" << std::endl;
		else
		{
			std::cout
				<< "|{"
				<< (aNode->left ? "/" : "_")
				<< (aNode->right ? "\\" : "_")
				<< "} size:"
				<< std::setw(3)
				<< std::setiosflags(std::ios::right)
				<< aNode->size
				<< " data:["
				<< std::hex;
				//<< aNode->data
				for(const auto i : aNode->data)
				{
					std::cout << +static_cast<uint8_t>(i) << ", ";
				}
				std::cout << "\b\b]|\r\n";

			if(aNewLine)
				std::cout << std::endl;
		}
	}

	void inorder(node* aNode) const
	{
		if(aNode == nullptr)
			return;

		inorder(aNode->left);
		display(aNode);
		inorder(aNode->right);
	}

	node* findBy(node* aNode, uint64_t aIndex) const
	{
		if(nullptr == aNode)
			return nullptr;

		const auto p = (nullptr != aNode->left) ? aNode->left->size + 1 : 1;

		if(aIndex == p)
			return aNode;
		else if(aIndex < p && nullptr != aNode->left)
			return findBy(aNode->left, aIndex);
		else if(aIndex > p && nullptr != aNode->right)
			return findBy(aNode->right, aIndex - p);

		return nullptr;
	}

public:
	OrderStatisticTree()
		: root{nullptr}
	{
	}

	~OrderStatisticTree()
	{
		root = makeEmpty(root);
	}

	void insert(T aData)
	{
		root = insert(aData, root);
	}

	void removeBy(uint64_t aIndex)
	{
		removeBy(aIndex + 1, root);
	}

	void display() const
	{
		inorder(root);
		std::cout << std::endl;
	}

	T& searchBy(uint64_t aIndex) const
	{
		assert(root->size >= aIndex + 1);
		return findBy(root, aIndex + 1)->data;
	}
};
}